package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/webview/webview"
)

func main() {
	program := filepath.Base(os.Args[0])
	args := os.Args[1:]
	if len(args) < 1 {
		fmt.Printf("uso: %s url\n", program)
	} else {
		url := args[0]
		fmt.Println("navegando %s", url)
		debug := true
		w := webview.New(debug)
		defer w.Destroy()
		w.SetTitle("Minimal webview example")
		w.SetSize(800, 600, webview.HintNone)
		w.Navigate(url)
		w.Run()
	}

}
