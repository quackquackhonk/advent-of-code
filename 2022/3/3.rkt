#lang racket

(define (run fnam)
  (call-with-input-file fnam rucksacks))

(define (rucksacks in)
  (define sack-strs
    (for/list ([l (in-lines in)])
      l))
  ;; (define common-items (map common-item sacks))
  ;; (define priorites (map priority common-items))
  (define groups (split-by-size 3 sack-strs))
  (define badges (map group-common-item groups))
  (define priorities (map priority badges))
  (foldl + 0 priorities))

;; a Rucksack is a list of list of characters.
;; Each list has the same length
;; the outer list has minimum 2 elements

;; turns a string `str' into a rucksack with `n' compartments
;; Assumes s can be split into n equally-sized compartments
(define (string->n-rucksack str n)
  (let* ([items (string->list str)] [num-items (length items)] [comp-size (/ num-items n)])
    (split-by-size comp-size items)))

(define (string->2-rucksack str)
  (string->n-rucksack str 2))

(define (split-by-size size items)
  (define (split-by-acc size items acc)
    (if (empty? items) acc (split-by-acc size (drop items size) (cons (take items size) acc))))
  (split-by-acc size items '()))

;; Returns a list of characters where each character is present in
;; every rucksack compartment
(define (common-items sack)
  (foldl list-common (first sack) (rest sack)))

;; given a list of rucksacks, finds the item common between all rucksacks
(define (group-common-item group)
  (first (common-items (map string->list group))))

;; given two lists, returns a list of all elements in l1 that are also
;; in l2
(define (list-common l1 l2)
  (filter (lambda (v) (member v l2)) l1))

(define (in-common common)
  (lambda (l) (list-common l common)))

(define (total-priority common sack)
  (define all-common (flatten (map (in-common common) sack)))
  (foldl + 0 (map priority all-common)))

(define (priority item)
  (let ([item-str (list->string (list item))])
    (cond
      [(regexp-match #rx"[a-z]" item-str) (- (char->integer item) 96)]
      [(regexp-match #rx"[A-Z]" item-str) (- (char->integer item) 38)])))
