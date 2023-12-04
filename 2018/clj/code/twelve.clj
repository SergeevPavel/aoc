(ns code.twelve)

(def state-str "#.##.###.#.##...##..#..##....#.#.#.#.##....##..#..####..###.####.##.#..#...#..######.#.....#..##...#")

(def rules-string (clojure.string/split ".#.#. => .
...#. => #
..##. => .
....# => .
##.#. => #
.##.# => #
.#### => #
#.#.# => #
#..#. => #
##..# => .
##### => .
...## => .
.#... => .
###.. => #
#..## => .
#...# => .
.#..# => #
.#.## => .
#.#.. => #
..... => .
####. => .
##.## => #
..### => #
#.... => .
###.# => .
.##.. => #
#.### => #
..#.# => .
.###. => #
##... => #
#.##. => #
..#.. => #" #"\n"))

(count rules-string)

(def rules (into {} (map (fn [s] (clojure.string/split s #" => "))
                         rules-string)))

(def state (into {} (map-indexed (fn [idx v] [idx (str v)]) state-str)))

(defn step [state]
  (let [min-key (apply min (keys state))
        max-key (apply max (keys state))]
    (into {}
          (keep (fn [idx]
                  (let [env (apply str (map #(get state % ".") (range (- idx 2) (+ idx 3))))]
                    (when (= (get rules env) "#")
                      [idx "#"]))))
          (range (- min-key 2) (+ max-key 3)))))

(defn result [state]
  (apply + (keys (filter (fn [[idx v]] (= v "#")) state))))

()

(def fs (take 500 (iterate step state)))
(def results (map result fs))
(map - (drop 1 results) results)
(last fs)

(def bn 50000000000)
(count "50000000000")
(+ (* 62 (- bn 499)) (last results))

(time (doseq [_ (range (/ bn 10))]
  (+ 2 2)))
