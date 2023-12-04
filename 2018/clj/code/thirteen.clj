(ns code.thirteen)

;(def input (line-seq (clojure.java.io/reader "/Users/pavel/hobby/adventofcode/code/resources/foo.txt")))
(def input (line-seq (clojure.java.io/reader "/Users/pavel/hobby/adventofcode/code/resources/day13.txt")))

(def state (into {}
                 (filter (fn [[k v]] (not= v \space)))
                 (mapcat (fn [[y s]]
                           (map-indexed (fn [x v]
                                          [[x y] v])
                                        s))
                         (map-indexed vector input))))

(defn retrive-ships [state]
  (let [ships (filter (fn [[k v]] (contains? #{\< \> \^ \v} v)) state)]
    {:track (reduce (fn [state [p dir]]
                      (if (contains? #{\< \>} dir)
                        (assoc state p \-)
                        (assoc state p \|)))
                    state
                    ships)
     :ships (map (fn [[p dir]] {:p p :dir dir :counter 0}) ships)}))

(let [{:keys [track ships]} (retrive-ships state)]
  (def track track)
  (def initial-ships ships))

(defn move [[x y :as p] dir]
  (case dir
    \> [(inc x) y]
    \< [(dec x) y]
    \^ [x (dec y)]
    \v [x (inc y)]))

(defn one-ship-step [{:keys [p dir counter]}]
  (let [next-point (move p dir)
        next-sym (get track next-point)]
    (merge {:p next-point}
           (case [dir next-sym]
             ([\> \-] [\< \-] [\^ \|] [\v \|]) {:dir dir :counter counter}
             [\> \\]                           {:dir \v :counter counter}
             [\^ \\]                           {:dir \< :counter counter}
             [\< \\]                           {:dir \^ :counter counter}
             [\v \\]                           {:dir \> :counter counter}

             [\> \/]                           {:dir \^ :counter counter}
             [\v \/]                           {:dir \< :counter counter}
             [\< \/]                           {:dir \v :counter counter}
             [\^ \/]                           {:dir \> :counter counter}

             [\^ \+]                           {:dir (nth [\< \^ \>] (mod counter 3))
                                                :counter (inc counter)}
             [\> \+]                           {:dir (nth [\^ \> \v] (mod counter 3))
                                                :counter (inc counter)}
             [\v \+]                           {:dir (nth [\> \v \<] (mod counter 3))
                                                :counter (inc counter)}
             [\< \+]                           {:dir (nth [\v \< \^] (mod counter 3))
                                                :counter (inc counter)}))))

(defn all-step [ships]
  (loop [moved []
         not-moved (sort-by (comp vec reverse :p) ships)]
    (if-let [sh (first not-moved)]
      (let [sh' (one-ship-step sh)
            crushed (into #{} (concat (filter #(= (:p sh') (:p %)) moved)
                                      (filter #(= (:p sh') (:p %)) not-moved)))]
        (if (not-empty crushed)
          (recur (filter #(not (contains? crushed %)) moved)
                 (filter #(not (contains? crushed %)) (next not-moved)))
          (recur (conj moved sh')
                 (next not-moved))))
      moved)))

(loop [ships initial-ships c 0]
  (if (or (<= (count ships) 1) (< 100000 c))
    [c ships]
    (recur (all-step ships) (inc c))))
