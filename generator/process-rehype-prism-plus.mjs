import { unified } from "unified";
import rehypeParse from "rehype-parse";
import rehypePrismPlus from "rehype-prism-plus";
import rehypeStringify from "rehype-stringify";

const processor = unified()
	.use(rehypeParse)
	.use(rehypePrismPlus)
	.use(rehypeStringify);

process.stdin.setEncoding("utf8");
process.stdin.on("data", (htmlInput) => {
	processor
		.process(htmlInput)
		.then((file) => {
			console.log(file.toString());
		})
		.catch((err) => {
			console.error("Error processing HTML:", err);
			process.exit(1);
		});
});
