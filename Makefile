TOPTARGETS := all clean

SUBDIRS := mmio usb shared init kernel boot

$(TOPTARGETS): $(SUBDIRS)
$(SUBDIRS):
	$(MAKE) -C $@ $(MAKECMDGOALS)

.PHONY: $(TOPTARGETS) $(SUBDIRS) kernel