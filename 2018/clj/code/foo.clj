(ns code.foo)

;; 1.1

(def numbers (line-seq (clojure.java.io/reader "/Users/pavel/work/foo.txt")))

(apply + (map #(Long/parseLong %)  numbers))

;; 1.2

(def numbers (map #(Long/parseLong %)  (line-seq (clojure.java.io/reader "/Users/pavel/work/foo1.txt"))))


(def fs (reductions + numbers))


(loop [tail (cycle numbers)
       prev #{}
       fr 0]
  (if (contains? prev fr)
    [fr (count prev)]
    (do
      (recur (rest tail)
             (conj prev fr)
             (+ fr (first tail))))))

;; 2.1


(def boxes (line-seq (clojure.java.io/reader "/Users/pavel/work/foo3.txt")))

(def id (first boxes))

(count boxes)
(def foo (count (filter (fn [id] (contains?  (into #{} (map second (frequencies id))) 2)) boxes)))
(def bar (count (filter (fn [id] (contains?  (into #{} (map second (frequencies id))) 3)) boxes)))

(filter (fn [id] (contains?  (into #{} (map second (frequencies id))) 3)) boxes)

(* foo bar)

;; 2.2
(frequencies (map count boxes))



(def ds (for [b1 boxes
           b2 boxes]
       {:b1 b1
        :b2 b2
        :diff (get (frequencies (map - (map int b1) (map int b2))) 0)}))

(filter #(= 25 (:diff %)) ds)

;; 3.1

(def input (line-seq (clojure.java.io/reader "/Users/pavel/work/foo3.txt")))

(count input)

(defn parse-claim [s]
  (let [[_ id x y w h] (re-matches #"(\S*) @ (\S*),(\S*): (\S*)x(\S*).*" s)
        result {:id id :x (Long/parseLong x) :y (Long/parseLong y) :w (Long/parseLong w) :h (Long/parseLong h)}]
    (assert (every? some? (map second result)))
    result))

(def input [
     "#1 @ 1,3: 4x4",
     "#2 @ 3,1: 4x4",
     "#3 @ 5,5: 2x2",
     "#4 @ 3,3: 2x2",
 ])

(def claims (mapv parse-claim input))

(def min-x (reduce (fn [acc cl]
                     (min (:x cl) acc))
                   (:x (first claims))
                   claims))

(def min-y (reduce (fn [acc cl]
                     (min (:y cl) acc))
                   (:y (first claims))
                   claims))

(def max-x (reduce (fn [acc {:keys [x w]}]
                     (max (+ x w) acc))
                   0
                   claims))

(def max-y (reduce (fn [acc {:keys [y h]}]
                     (max (+ y h) acc))
                   0
                   claims))

(reduce (fn [acc {:keys [w]}]
                     (max w acc))
                   0
                   claims)

(reduce (fn [acc {:keys [h]}]
                     (max h acc))
                   0
                   claims)

(def mw 1010)
(def mh 1010)

(def initial (into [] (repeat (* mw mh) 0)))

(count initial)

(def result (reduce (fn [acc {:keys [x y w h]}]
                      (reduce (fn [acc {:keys [i j]}]
                                (update acc (+ (* mh i) j) inc))
                              acc
                              (for [i (range x (+ x w))
                                    j (range y (+ y h))] {:i i :j j})))
                    initial
                    claims))

(reduce (fn [acc {:keys [id x y w h]}]
          (when (= (* w h) (apply + (map (fn [{:keys [i j]}]
                                           (get acc (+ (* mh i) j) inc))
                                         (for [i (range x (+ x w))
                                               j (range y (+ y h))] {:i i :j j}))))
            (prn id))
          acc)
        result
        claims)

(->> (frequencies result)
     (map second)
     (apply +))

(* mw mh)

(->> result
     (frequencies)
     (filter #(< 1 (first %)))
     (map second)
     (apply +))

;; 119551

(frequencies result)

;; 4.1


(def input (line-seq (clojure.java.io/reader "/Users/pavel/hobby/adventofcode/code/resources/input.txt")))

(def s "[1518-07-14 00:55] wakes up")

(defn parse-record [s]
  (let [[_ year month day hour minute action] (re-matches #"\[(\S*)-(\S*)-(\S*) (\S*):(\S*)\] (.*)" s)
        action (cond
                 (re-matches #"wakes up" action) {:action :wakes}
                 (re-matches #"falls asleep" action) {:action :falls}
                 (re-matches #"Guard (\S*) begins shift" action) {:action :start
                                                                  :guard-id (second (re-matches #"Guard (\S*) begins shift" action))})
        result (merge {:year (Long/parseLong year)
                       :month (Long/parseLong month)
                       :day (Long/parseLong day)
                       :hour (Long/parseLong hour)
                       :minute (Long/parseLong minute)}
                      action)]
    (assert (every? some? (map second result)))
    result))

(def records (mapv parse-record input))

(def sorted-records (sort-by (juxt :year :month :day :hour :minute)
                             records))

(def guard-actions-list
  (loop [result []
         current nil
         rest sorted-records]
    (if-let [{:keys [action guard-id] :as next-action} (first rest)]
      (if (= action :start)
        (recur
          (if (some? current)
            (conj result current)
            result)
          {:guard-id guard-id
           :actions []}
          (next rest))
        (recur
          result
          (update current :actions conj next-action)
          (next rest)))
      (conj result current))))


(assert (= (count sorted-records)
           (apply + (map (fn [{:keys [actions]}] (inc (count actions))) guard-actions-list))))


(def bad-actions (->> guard-actions-list
                      (map :actions)
                      (filter (fn [actions]
                                (or
                                  (not= 0 (rem (count actions) 2))
                                  (some not (mapcat (fn [[{falls-action :action} {wakes-action :action}]]
                                                      [(= falls-action :falls)
                                                       (= wakes-action :wakes)])
                                                    (partition 2 actions))))))))


(defn slept-time [guard-actions]
  (->> guard-actions
       (partition 2)
       (map (fn [[falls-action wakes-action]]
              (assert (= 0 (:hour wakes-action)))
              (- (:minute wakes-action) (:minute falls-action))))
       (apply +)))

(->> (mapv (fn [{:keys [actions guard-id]}]
             {guard-id (slept-time actions)}) guard-actions-list)
     (apply merge-with +)
     (sort-by second))

(def target-guard-id "#2381")

(def target-guard (filter (fn [{:keys [guard-id]}] (= guard-id target-guard-id)) guard-actions-list))

(def initial-range (into [] (repeat 60 0)))

(defn apply-hour [r actions]
  (reduce (fn [r {:keys [from to]}]
            (map-indexed (fn [idx v] (if (and (<= from idx)
                                              (< idx to))
                                       (inc v)
                                       v))
                         r))
          r
          (->> actions
               (partition 2)
               (map (fn [[falls-action wakes-action]]
                      (assert (= 0 (:hour wakes-action)) wakes-action)
                      {:from (:minute falls-action)
                       :to (:minute wakes-action)})))))


;; 44

(count target-guard)

(* 44 2381)

(map-indexed vector (reduce apply-hour initial-range (map :actions target-guard)))

;; 4.2

(->> (reduce (fn [acc {:keys [guard-id actions] :as guard}]
               (let [r (or (get acc guard-id) initial-range)]
                 (assoc acc guard-id (apply-hour r actions))))
             {}
             guard-actions-list)
     (map (fn [[k v]] [k (last (sort-by second (map-indexed vector v)))])))

(* 3137 41)

