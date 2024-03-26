#!/bin/sh

denofmt() {
	which deno | fgrep -q deno || return
	deno fmt
}

sh_fmt() {
	which shfmt | fgrep -q shfmt || return
	shfmt --write *.sh
}

denofmt
sh_fmt
