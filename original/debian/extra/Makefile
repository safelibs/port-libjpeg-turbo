INSTALL = install -m755
INSTALLDIR = install -m755 -d
DESTDIR =
prefix  = /usr/local
bindir  = $(prefix)/bin
mandir  = $(prefix)/share/man/man1

all: jpegexiforient

jpegexiforient: jpegexiforient.c
	$(CC) $(CFLAGS) $(CPPFLAGS) $(LDFLAGS) -o jpegexiforient jpegexiforient.c
clean:
	-rm -f jpegexiforient
install:
	$(INSTALLDIR) $(DESTDIR)$(bindir)
	$(INSTALLDIR) $(DESTDIR)$(mandir)
	$(INSTALL) jpegexiforient    $(DESTDIR)$(bindir)
	$(INSTALL) jpegexiforient.1  $(DESTDIR)$(mandir)
	$(INSTALL) exifautotran      $(DESTDIR)$(bindir)
	$(INSTALL) exifautotran.1    $(DESTDIR)$(mandir)
