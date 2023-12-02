#lang racket

(define (run fname)
  (call-with-input-file fname find-space))

(define (find-space in)
  (let* ([fs (make-fs in)] [small-dirs (map small-dir? fs)]) (dir-size fs)))

;; A file system is a list of entries

;; an entry is one of
;; file-entry with a name and size
;; dir-entry with a name and contents
(struct file-entry (name size))
(struct dir-entry (name contents))

(define (make-fs in)
  'todo)

(define (small-dir? ent)
  'todo)

(define (dir-size dir)
  'todo)
