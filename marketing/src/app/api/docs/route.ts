import { ScalarApiReference } from "@scalar/nextjs";

export const GET = ScalarApiReference({
  spec: {
    url: "http://localhost:3000/openapi.json",
  },
  pageTitle: "Irori API Documentation",
  metaData: {
    title: "Irori API Docs",
    description: "Interactive API documentation for Irori",
  },
});
