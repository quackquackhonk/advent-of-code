#lang racket

(define (run fname)
  (call-with-input-file fname camp-cleanup))

(define (camp-cleanup in)
  (define pairs
    (for/list ([l (in-lines in)])
      (string->assn-pair l)))
  (length (filter overlap? pairs)))

;; an assignment is a range of numbers from lo to hi
(struct assignment (lo hi))

(define (string->assn-pair s)
  (let ([assn-strs (string-split s ",")])
    (cons (string->assn (first assn-strs)) (string->assn (second assn-strs)))))

(define (string->assn s)
  (let ([bounds (string-split s "-")])
    (assignment (string->number (first bounds)) (string->number (second bounds)))))

;; does either a1 or a2 fully contain the other assignment?
(define (either-contains? a-pair)
  (let ([left (car a-pair)] [right (cdr a-pair)])
    (or (assignment-contains? left right) (assignment-contains? right left))))

;; does a1 fully contain a2?
(define (assignment-contains? a1 a2)
  (and (<= (assignment-lo a1) (assignment-lo a2)) (>= (assignment-hi a1) (assignment-hi a2))))

(define (overlap? a-pair)
  (let* ([left (car a-pair)]
         [right (cdr a-pair)]
         [l-lo (assignment-lo left)]
         [l-hi (assignment-hi left)]
         [r-lo (assignment-lo right)]
         [r-hi (assignment-hi right)])
    (not (or (> l-lo r-hi) (> r-lo l-hi)))))
