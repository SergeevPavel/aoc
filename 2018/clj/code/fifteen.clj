(ns code.fifteen
  (:require [lanterna.screen :as s]))

(def input (line-seq (clojure.java.io/reader "./resources/day15.txt")))

(defn input->state [input]
  (let [field (->> input
                   (map-indexed vector)
                   (mapcat (fn [[y line]]
                             (map-indexed (fn [x s]
                                            (let [cell (case s
                                                         \G {:type       :goblin
                                                             :hit-points 200
                                                             :attack     3}
                                                         \E {:type       :elf
                                                             :hit-points 200
                                                             :attack     3}
                                                         \# {:type :wall}
                                                         \. {:type :cavern})]
                                              [[x y] cell]))
                                          line)))
                   (into (sorted-map)))
        field-keys (keys field)
        width (reduce max 0 (map first field-keys))
        height (reduce max 0 (map second field-keys))]
    {:field  field
     :width  width
     :height height}))

(defn cell->sym [cell]
  (case (:type cell)
    :goblin "G"
    :elf "E"
    :wall "#"
    :cavern "."))

(defn cell->stat [cell]
  (case (:type cell)
    :goblin (str "G(" (:hit-points cell) ")")
    :elf (str "E(" (:hit-points cell) ")")
    nil))

(defn show-state [scr {:keys [field width height]}]
  (let [base-x 3
        base-y 3]
    (do
      (doseq [[[x y] cell] field]
        (s/put-string scr (+ x base-x) (+ y base-y) (cell->sym cell)))
      (doseq [j (range height)]
        (let [stats-line (->> field
                              (filter (fn [[[x y] cell]] (= y j)))
                              (sort-by (fn [[[x y] cell]] x))
                              (keep (fn [[[x y] cell]] (cell->stat cell)))
                              (interpose ", ")
                              (apply str))]
          (s/put-string scr (+ width 3 base-x) (+ j base-y) stats-line)))
      (s/redraw scr))))

(defn neighbors [[x y :as p]]
  [[x (dec y)]
   [(dec x) y]
   [(inc x) y]
   [x (inc y)]])

(defn enemy-race [type]
  (case type
    :elf :goblin
    :goblin :elf))

(defn unit->pos [[[x y :as pos] {:keys [type hit-points attack]} :as unit]]
  pos)

(defn unit->hit-points [[[x y :as pos] {:keys [type hit-points attack]} :as unit]]
  hit-points)

(defn unit->race [[[x y :as pos] {:keys [type hit-points attack]} :as unit]]
  type)

(defn targets-in-range [{:keys [field] :as state} unit]
  (let [er (enemy-race (unit->race unit))]
    (->> (neighbors (unit->pos unit))
         (keep (fn [n]
                 (when-let [neighbour-cell (get field n)]
                   (when (= er (:type neighbour-cell))
                     [n neighbour-cell])))))))

(defn min-by-key [f coll]
  (when (not-empty coll)
    (reduce (fn [a b]
              (if (<= 0 (compare (f a) (f b))) b a))
            coll)))

(defn target-to-attack [state unit]
  (->> (targets-in-range state unit)
       (min-by-key (fn [[[x y] {:keys [hit-points]}]]
                     [hit-points y x]))))

(defn- do-attack [state from to]
  (let [a (get-in state [:field from :attack])]
    (update-in state [:field to] (fn [{:keys [hit-points] :as cell}]
                                   (let [new-hp (- hit-points a)]
                                     (if (pos? new-hp)
                                       (assoc cell :hit-points new-hp)
                                       {:type :cavern}))))))

(defn attack [state [p :as unit]]
  (let [unit-to-attack (target-to-attack state unit)]
    (if (some? unit-to-attack)
      (do-attack state p (unit->pos unit-to-attack))
      state)))

(defn bfs [{:keys [field] :as state} start-point]
  (loop [visited {}
         step 0
         front #{start-point}]
    (if (empty? front)
      visited
      (recur (into visited (map (fn [p] [p step]) front))
             (inc step)
             (into #{}
                   (filter (fn [p]
                             (and (= :cavern (get-in field [p :type]))
                                  (not (contains? visited p))))
                           (mapcat neighbors front)))))))

(defn all-targets [{:keys [field] :as state} unit]
  (let [er (enemy-race (unit->race unit))]
    (filter (fn [enemy-unit] (= er (unit->race enemy-unit))) field)))

(defn points-to-go [{:keys [field] :as state} unit]
  (let [targets (all-targets state unit)]
    (->> targets
         (map first)
         (mapcat neighbors)
         (filter (fn [p]
                   (or (= :cavern (get-in field [p :type]))
                       (= p (unit->pos unit)))))
         (into #{}))))

(defn spy [msg v]
  (prn msg v)
  v)

(defn shortest-path [state dists source destination]
  (loop [path []
         current destination]
    (if (= current source)
      (reverse path)
      (recur (conj path current)
             (->> (neighbors current)
                  (keep (fn [p]
                          (when-let [d (dists p)]
                            [p d])))
                  (min-by-key (fn [[[x y] d]] [d y x]))
                  (first))))))

(defn point-to-move [state current-point target-point]
  (if (= current-point target-point)
    current-point
    (let [dists (bfs state target-point)]
      (some->> (neighbors current-point)
               (keep (fn [p]
                       (when-let [d (dists p)]
                         [p d])))
               (min-by-key (fn [[[x y] d]] [d y x]))
               (first)))))

(defn- do-move [state from to]
  (if (= from to)
    state
    (-> state
        (assoc-in [:field to] (get-in state [:field from]))
        (assoc-in [:field from] {:type :cavern}))))

(defn move [state [p {:keys [type hit-points attack] :as stats} :as unit]]
  (let [dists (bfs state p)
        next-p (some->> (points-to-go state unit)
                        (keep (fn [p]
                                (when-let [d (dists p)]
                                  [p d])))
                        (min-by-key (fn [[[x y] d]] [d y x]))
                        (first)
                        (point-to-move state p))]
    (if (some? next-p)
      [(do-move state p next-p) [next-p stats]]
      [state unit])))

(defn take-turn [state [p {:keys [type]} :as unit]]
  (if (contains? #{:elf :goblin} type)
    (let [[new-state new-unit] (move state unit)]
      (attack new-state new-unit))
    state))

(defn turn [scr {:keys [field] :as state}]
  (let [unit-points (->> field
                         (keep (fn [[p cell]]
                                 (when (contains? #{:elf :goblin} (:type cell))
                                   p)))
                         (sort-by (fn [[x y]] [y x])))]
    (reduce (fn [state p]
              (let [unit [p (get-in state [:field p])]
                    next-state (take-turn state unit)]
                (show-state scr next-state)
                (s/get-key-blocking scr)
                next-state))
            state
            unit-points)))

(defn all-hp [{:keys [field] :as state}]
  (->> field
       (keep (fn [[p {:keys [hit-points]}]] hit-points))
       (reduce + 0)))

(defn run! [input]
  (let [initial-state (input->state input)
        scr (s/get-screen :swing {:cols 160 :rows 120})]
    (s/in-screen scr
                 (loop [state initial-state
                        step 0]
                   (s/clear scr)
                   (s/put-string scr 0 0 (str "step: " step " hp: " (all-hp state) " score: " (* step (all-hp state))))
                   (show-state scr state)
                   (when (not= :escape (s/get-key-blocking scr))
                     (let [new-state (turn scr state)]
                       (if (= new-state state)
                         (recur state step)
                         (recur new-state (inc step)))))))))

(comment

  (run! (line-seq (clojure.java.io/reader "./resources/day15.txt")))
  (use 'clojure.stacktrace)
  (print-stack-trace *e 10)

  (input->state input)
  (s/stop *1)
  )


