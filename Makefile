.PHONY: main

main:
	@if [ $$(id -u) -ne 0 ]; then \
		echo "Please, run as superuser. Exit."; \
		exit 1; \
	fi

	@echo "Building HTTP Proxy util."

	@if [ "$(sh -c \"cargo --help)\"" = "" ]; then \
		echo "Cannot find Cargo. Exit."; \
		exit 1; \
	fi 

	@cargo build

	@echo "Done. Copying to /usr/bin/..."
	@cp ./target/debug/http_proxy /usr/bin/http-proxy

	clear
	@echo " _   _  ____  ____  ____    ____  ____  _____  _  _  _  _ "
	@echo "( )_( )(_  _)(_  _)(  _ \  (  _ \(  _ \(  _  )( \/ )( \/ )"
	@echo " ) _ (   )(    )(   )___/   )___/ )   / )(_)(  )  (  \  / "
	@echo "(_) (_) (__)  (__) (__)    (__)  (_)\_)(_____)(_/\_) (__) "
	@echo ""
	@echo ""
	@echo "Done. Now you can use it from \"http-proxy\" command."
	@echo "Try: http-proxy --help for more info."
