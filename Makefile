TYPES_DIR=lib/kube-types
TYPES_SRC=$(TYPES_DIR)/src
SWAGGER_JSON=$(TYPES_DIR)/swagger.json

help:
	$(info Targets)
	$(info -----------------------------------------------------------------------)
	$(info kube-types      | rebuilds all kubernetes types based on the latest spec)

$(SWAGGER_JSON):
	mkdir -p $(dir $@)
	curl --fail -sLo $@ https://raw.githubusercontent.com/kubernetes/kubernetes/master/api/openapi-spec/swagger.json

kube-types: $(SWAGGER_JSON)
	rm -Rf $(TYPES_SRC)
	mkdir -p $(TYPES_SRC)
	cd lib/ext/k8s-openapi-codegen && cargo +nightly run -- $(CURDIR)/$< $(CURDIR)/$(TYPES_SRC)
	cp Cargo.toml.types $(dir $(TYPES_SRC))/Cargo.toml
