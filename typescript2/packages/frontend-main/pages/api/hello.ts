// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from "next";
import { awesomeFn } from "@oyelowo/libraries-core";

type Data = {
  name: string;
  greetings: string;
};

export default function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  // dependencies across child packages
  const out = awesomeFn();

  res.status(200).json({ name: "John Doe", greetings: out });
}
