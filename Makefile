resources/symphony-crd.yaml:
	cargo run --bin resource-builder

apply/items:
	mkdir -p apply
	touch apply/items

apply/crd:
	kubectl apply -f resources/symphony-crd.yaml
	touch apply/crd 

clean:
	rm -rf resources/