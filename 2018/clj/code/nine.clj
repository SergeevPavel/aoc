(ns code.nine)



;10 players; last marble is worth 1618 points: high score is 8317
;13 players; last marble is worth 7999 points: high score is 146373
;17 players; last marble is worth 1104 points: high score is 2764
;21 players; last marble is worth 6111 points: high score is 54718
;30 players; last marble is worth 5807 points: high score is 37305

;
;(defn mleft [{:keys [left center right] :as st}]
;  (if (pos? (count left))
;    {:left (pop left)
;     :center (last left)
;     :right (conj right center)}
;    (if (pos? (count right))
;      {:left (into [center] (for [idx (range (dec (count right)) 0 -1)]
;                              (nth right idx)))
;       :center (first right)
;       :right []}
;      st)))
;
;(defn mright [{:keys [left center right] :as st}]
;  (if (pos? (count right))
;    {:left (conj left center)
;     :center (last right)
;     :right (pop right)}
;    (if (pos? (count left))
;      {:left []
;       :center (first left)
;       :right (into [center] (for [idx (range (dec (count left)) 0 -1)]
;                              (nth left idx)))}
;      st)))
;
;(defn insert [{:keys [left center right] :as st} v]
;  (assoc st :right (conj right v)))
;
;(defn delete [{:keys [left center right] :as st}]
;  (if (pos? (count right))
;    (assoc st :center (last right)
;              :right (pop right))
;    {:left []
;     :center (first left)
;     :right (vec (for [idx (range (dec (count right)) 0 -1)]
;                              (nth right idx)))}))
;
;
;(def initial-state {:left [] :center 0 :right []})
;
;(defn norm [{:keys [left center right] :as st}]
;  (vec (concat left [center] (reverse right))))
;
;(norm initial-state)
;
;(mright initial-state)


(defn initial-state []
  (let [*n (volatile! nil)]
    (vreset! *n {:left *n :val 0 :right *n})
    *n))

(def node (initial-state))

(defn mleft [*node]
  (:left @*node))

(defn mright [*node]
  (:right @*node))

(defn insert [*node v]
  (let [*right (:right @*node)
        *new-node (volatile! {:left *node :val v :right *right})]
    (vswap! *node assoc :right *new-node)
    (vswap! *right assoc :left *new-node)
    *new-node))

(defn delete [*node]
  (let [*left (:left @*node)
        *right (:right @*node)]
    (vswap! *left assoc :right *right)
    (vswap! *right assoc :left *left)
    *right))

(defn current [*node]
  (:val @*node))

(defn solve [players last-marble]
  (loop [state (initial-state)
         step 1
         scores (vec (repeat players 0))]
    (if (<= step last-marble)
        (if (= 0 (rem step 23))
          (let [player-id (rem step players)
                state' (last (take 8 (iterate mleft state)))
                scores' (update scores player-id + step (current state'))]
            (recur (delete state') (inc step) scores'))
          (recur (insert (mright state) step) (inc step) scores))
        (apply max scores))))


;
;(last (take 1 (iterate mleft foo)))

;(solve 9 25)

;10 players; last marble is worth 1618 points: high score is 8317
;13 players; last marble is worth 7999 points: high score is 146373
;17 players; last marble is worth 1104 points: high score is 2764
;21 players; last marble is worth 6111 points: high score is 54718
;30 players; last marble is worth 5807 points: high score is 37305

;; 426 players; last marble is worth 72058 points =>> ???

;(solve 10 1618)
;897.289419 msecs
(time (solve 13 7999))
;(solve 17 1104)
;(solve 21 6111)
;(solve 30 5807)
;(time (solve 426 72058)) ;; 46816
(solve 426 (* 100 72058))

