package main

import (
	"time"
)

type File struct {
	name      string
	size      int64
	directory bool

	// Entries is a map of file names to file info
	Entries map[string]*File
}

func (f *File) AddFile(name string, size int64, directory bool) {
	file := &File{name: name, size: size, directory: directory}
	if f.Entries == nil {
		f.Entries = make(map[string]*File)
	}
	f.Entries[name] = file
}

func (f *File) Name() string {
	return f.name
}

func (f *File) Size() int64 {
	return f.size
}

func (f *File) ModTime() time.Time {
	return time.Now()
}

func (f *File) IsDir() bool {
	return f.directory
}

func (f *File) Sys() interface{} {
	return nil
}
