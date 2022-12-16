#lang racket

(define (run fname)
  (call-with-input-file fname total-score))

(define (total-score in)
  (foldr + 0 (scores in)))

(define (scores in)
  (define (process line)
    (string-split line))
  (for/list ([line (in-lines in)])
    (score (process line))))

(define (score strat)
  (let ([opp (first strat)] [outcome (second strat)])
    (+ (result-score outcome) (choice-score (find-move opp outcome)))))

(define (result-score outcome)
  (cond
    [(winner? outcome) 6]
    [(tie? outcome) 3]
    [(loser? outcome) 0]))

(define (choice-score c)
  (cond
    [(rock? c) 1]
    [(paper? c) 2]
    [(scissors? c) 3]))

(define (find-move opp outcome)
  (cond
    [(winner? outcome) (find-win opp)]
    [(tie? outcome) opp]
    [(loser? outcome) (find-lose opp)]))

(define (find-win opp)
  (cond
    [(rock? opp) "B"]
    [(paper? opp) "C"]
    [(scissors? opp) "A"]))

(define (find-lose opp)
  (cond
    [(rock? opp) "C"]
    [(paper? opp) "A"]
    [(scissors? opp) "B"]))

(define (loser? strat)
  (string=? strat "X"))
(define (tie? strat)
  (string=? strat "Y"))
(define (winner? strat)
  (string=? strat "Z"))

(define (rock? s)
  (string=? s "A"))

(define (paper? s)
  (string=? s "B"))

(define (scissors? s)
  (string=? s "C"))
