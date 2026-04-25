import { test, expect } from "@playwright/test";
import YAML from "js-yaml";

import fs from "fs";
import path from "path";

type Profile = {
  name: string;
  images: (string | { url: string })[];
};

test("Bots' image urls intact", async ({ page }) => {
  // Read and parse the YAML file from the workspace
  const yamlPath = path.resolve(process.cwd(), "public", "bots.yaml");
  const raw = fs.readFileSync(yamlPath, "utf8");
  const profiles = YAML.load(raw) as Profile[];

  // Build a simple HTML page containing all images
  const html = `
		<!doctype html>
		<html>
			<head>
				<meta charset="utf-8" />
				<title>bots images</title>
				<style>img{max-width:200px;margin:4px}</style>
			</head>
			<body>
      ${profiles
        .reduce((arr, p) => {
          const tags = p.images
            .map((img, i) => ({
              src: typeof img == "string" ? img : img.url,
              alt: `${p.name}'s image #${i}`,
            }))
            .filter((i) => i.src.startsWith("https://"))
            .map(({ src, alt }) => `<img src="${src}" alt="${alt}">`);

          arr.push(...tags);

          return arr;
        }, [] as string[])
        .join("\n")}
			</body>
		</html>
	`;

  // Set the page content and wait for load (images included)
  await page.setContent(html, { waitUntil: "load" });

  // Evaluate whether images loaded successfully (naturalWidth > 0)
  const results = await page.evaluate(() => {
    return Array.from(document.images).map(({ src, alt, naturalWidth }) => ({
      src,
      alt,
      naturalWidth,
    }));
  });

  const failed: Pick<HTMLImageElement, "src" | "alt" | "naturalWidth">[] =
    results.filter((img) => !img.naturalWidth || img.naturalWidth === 0);

  if (failed.length > 0) {
    const failedImages = failed
      .map((img) => `${img.alt}\n\t${img.src}\n`)
      .join("\n");

    throw new Error(
      `Some images failed to load (${failed.length}):\n${failedImages}`,
    );
  }

  expect(failed.length).toBe(0);
});
