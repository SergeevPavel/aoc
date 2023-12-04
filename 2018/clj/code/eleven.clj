(ns code.eleven)

(def table
  (let [serial 1955
        f (fn [x y]
               (let [rack-id (+ x 10)
                     val (str (Math/abs (* rack-id (+ serial (* y rack-id)))))]
                 (- (Long/parseLong (str (nth val (- (count val) 3)))) 5)))]
    (into {}
          (map (fn [[i j]] [[i j] (f i j)]))
          (for [i (range 1 305) j (range 1 305)] [i j]))))

(defn sq [x y d]
  (reductions (fn [acc s]
                (let [hor (for [i (range (inc s))] (get table [(+ x i) (+ y s)]))
                      ver (for [j (range s)] (get table [(+ x s) (+ y j)]))]
                  (+ acc (apply + hor) (apply + ver))))
              0
              (range d)))

(reduce
 (fn [a b]
   (if (< (first a) (first b))
     b
     a))
 (mapcat (fn [[i j]]
           (let [sq-size (min (- 301 i) (- 301 j))]
             (map-indexed (fn [idx v]
                            [v i j idx])
                          (sq i j sq-size))))
         (for [i (range 1 300) j (range 1 300)] [i j])))
