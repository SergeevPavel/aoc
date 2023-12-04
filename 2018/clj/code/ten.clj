(ns code.ten
  (:require [clojure.string :refer [trim]]))

(def input (line-seq (clojure.java.io/reader "/Users/pavel/hobby/adventofcode/code/resources/day10.txt")))

(def s (first input))


(defn parse-line [s]
  (let [[_ x y vx vy] (re-matches #"position=<(.*), (.*)> velocity=<(.*), (.*)>" s)]
    {:pos [(Long/parseLong (trim x)) (Long/parseLong (trim y))]
     :vel [(Long/parseLong (trim vx)) (Long/parseLong (trim vy))]}))

(def points  (map parse-line input))

(defn step [{[x y] :pos [vx vy] :vel :as st}]
  (assoc st :pos [(+ x vx) (+ y vy)]))

(defn full-step [points]
  (mapv step points))

(defn bounds [points]
  (let [xs (map (comp first :pos) points)
        ys (map (comp second :pos) points)]
    [[(apply min xs) (apply min ys)] [(apply max xs) (apply max ys)]]))

(def r (drop 10000 (take 11000 (map-indexed vector (iterate full-step points)))))

(def l (apply str (repeat 140 ".")))
(def border-line (apply str (repeat 140 "=")))

(def xres 141)
(def yres 51)

(def origin (first (bounds points)))
(def points-set (set (map (fn [p] (map - p origin)) (map :pos points))))
(apply min (map second points-set))

(defn show [points]
  (let [[[minx miny :as origin] _] (bounds points)
        points-set (set (map (fn [p] (map - p origin)) (map :pos points)))]
    (prn border-line)
    (doseq [y (range yres)]
      (prn (apply str (for [x (range xres)]
                        (if (contains? points-set [x y])
                          \#
                          \.)))))
    (prn border-line)))


(def selected-frames (filter (fn [[idx v]] (< 10600 idx 10610)) r))

(map (fn [[idx points]]
       (bounds points))
     selected-frames)

(doseq [f selected-frames]
  (let [[idx points] f]
    (prn idx)
    (show points)))


(def points (second (second selected-frames)))
(show points)

(show (second (first selected-frames)))

;; 604-607

(first r)


(+ 2 2)
(count selected-frames)
