(ns code.five)


;; 5.1


(def input (drop-last (slurp "/Users/pavel/hobby/adventofcode/code/resources/day5.txt")))
(count input)

(defn kill? [x y]
  (and (some? x) (some? y) (= 32 (Math/abs (- (int x) (int y))))))

(def s "dabAcCaCBAcCcaDA")

(defn one-step [s]
  (loop [rst s
         res []]
    (if (seq rst)
      (let [[x y] rst]
        (if (kill? x y)
          (recur (drop 2 rst)
                 res)
          (recur (rest rst)
                 (conj res x))))
      res)))

(into #{} (drop-last input))

(one-step "Aa")

(defn full-reduction [s]
  (loop [m s]
    (let [m' (one-step m)]
      (if (= (count m') (count m))
        m
        (recur m')))))

(char (- 97 32))

;; 5.2

(->> (range (int \a) (inc (int \z)))
     (map (fn [k]
            (let [s' (filter (fn [l]
                               (and (not= l (char k))
                                    (not= l (char (- k 32)))))
                             input)]
              (count (full-reduction s'))))))

(count input)

(def rr (full-reduction input))

(count (apply str rr))
;; 9462

(count (one-step (apply str rr)))
(count rr)

;; dabAcCaCBAcCcaDA
;;  dabAcCaCBAcCcaDA
