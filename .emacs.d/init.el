;;; package --- Summary

;;; Commentary:
;;; This file runs when you first open Emacs.

;;; Code:
(setq gc-cons-threshold (* 50 1024 1024))

(package-initialize)
(setq package-enable-at-startup nil)


(add-to-list 'package-archives '("melpa" . "http://melpa.org/packages/"))
(package-initialize)


(unless (package-installed-p 'use-package)
  (package-refresh-contents)
  (package-install 'use-package))


(eval-when-compile (require 'use-package))
(setq use-package-always-ensure t)


(use-package evil :config (evil-mode 1))


(add-hook 'emacs-startup-hook
	  (lambda ()
	    (setq gc-cons-threshold (* 2 1024 1024))
	    (message "Emacs ready in %s with %d garbage collections."
		     (format "%.2f seconds"
			     (float-time
			      (time-subtract after-init-time before-init-time)))
		     gcs-done)))


(menu-bar-mode -1)
(when (display-graphic-p)
  (tool-bar-mode -1)
  (scroll-bar-mode -1))


(setq inhibit-splash-screen t)
(switch-to-buffer "**")


(setq make-backup-files nil)
(setq auto-save-default nil)


(defalias 'yes-or-no-p 'y-or-n-p)


(setq ring-bell-function 'ignore)


(use-package panda-theme)


(use-package linum-relative
  :config (linum-relative-global-mode)
  :custom (linum-relative-current-symbol ""))


(use-package company
  :hook (prog-mode . company-mode)
  :custom
  (company-idle-delay 0)
  (company-minimum-prefix-length 1))


(use-package which-key
  :defer 1
  :config (which-key-mode))


(use-package ivy
  :defer 1

  :config
  (ivy-mode 1)

  :custom
  (ivy-use-virtual-buffers t)
  (enable-recursive-minibuffers t))


(use-package flycheck
  :hook (prog-mode . flycheck-mode))


(use-package flymake
  :hook (python-mode . flymake-mode))


(use-package rainbow-delimiters
  :hook (prog-mode . rainbow-delimiters-mode)
  :custom (show-paren-delay 0)
  :config (show-paren-mode 1))


(use-package smartparens
  :hook (prog-mode . smartparens-mode)
  :commands sp-local-pair
  :config
  (sp-local-pair 'emacs-lisp-mode "'" nil :actions nil)
  (sp-local-pair 'emacs-lisp-mode "`" nil :actions nil))


(use-package evil-smartparens
  :hook (smartparens . evil-cleverparens-mode))


(use-package evil-cleverparens
  :hook (emacs-lisp-mode . evil-cleverparens-mode))


(use-package aggressive-indent
  :hook (emacs-lisp-mode . aggressive-indent-mode))


(use-package general)


(general-define-key
 :states 'normal
 :keymaps '(global dired-mode-map)
 :prefix "SPC"
 "f" 'find-file
 "x" 'execute-extended-command
 "w" '(nil :which-key "window")
 "b" '(nil :which-key "buffer")
 "s" 'shell)


(general-define-key
 :states 'normal
 :keymaps '(global dired-mode-map)
 :prefix "SPC w"
 "/" 'split-window-horizontally
 "-" 'split-window-vertically
 "m" 'delete-other-windows
 "c" 'evil-window-delete
 "h" 'evil-window-left
 "j" 'evil-window-down
 "k" 'evil-window-up
 "l" 'evil-window-right)


(defun kill-other-buffers ()
  "Kill all other buffers."
  (interactive)
  (mapc 'kill-buffer (delq (current-buffer) (buffer-list))))


(general-define-key
 :states 'normal
 :keymaps '(global dired-mode-map)
 :prefix "SPC b"
 "s" 'switch-to-buffer
 "l" 'list-buffers
 "o" 'kill-other-buffers
 "k" 'kill-buffer-and-window)


(general-define-key
 :keymaps 'emacs-lisp-mode-map
 :states 'normal
 :prefix ","
 "e" 'eval-defun)


(use-package adjust-parens
  :hook (emacs-lisp-mode . adjust-parens-mode))


(general-define-key
 :states 'insert
 :keymaps 'emacs-lisp-mode-map
 "TAB" 'lisp-indent-adjust-parens
 "<backtab>" 'lisp-dedent-adjust-parens)


(use-package rust-mode
  :mode "\\.rs\\'"
  :custom (rust-format-on-save t))


(use-package flycheck-rust
  :hook (rust-mode . flycheck-rust-mode)
  :config (flycheck-rust-setup))

(use-package toml-mode :mode "\\.toml\\'")

(use-package racer :hook (rust-mode . racer-mode))


(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(package-selected-packages
   (quote
    (racer toml-mode flycheck-rust rust-mode elpy adjust-parens general aggressive-indent evil-cleverparens evil-smartparens smartparens rainbow-delimiters flycheck ivy which-key company linum-relative panda-theme evil use-package))))

(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )

(provide 'init)
;;; init.el ends here
