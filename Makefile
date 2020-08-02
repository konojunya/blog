clean:
	rm -rf content
	mkdir content
	touch content/.gitkeep

serve:
	serve content
