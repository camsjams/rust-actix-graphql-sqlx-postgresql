import { ApolloServer } from "apollo-server";
import { ApolloGateway } from "@apollo/gateway";

const gateway = new ApolloGateway({
	serviceList: [
		{ name: "users", url: "http://localhost:8080/" },
		{ name: "skills", url: "http://localhost:8081/" },
	]
});

function main() {
	const server = new ApolloServer({
		gateway,
		engine: false,
		subscriptions: false,
	});

	return server.listen()
		.then(({ url }) => {
			console.log(`🚀 Gateway ready at ${url}`);
		})
}

main()
	.catch((error) => {
		console.error('Gateway startup failed:', error);
	});
