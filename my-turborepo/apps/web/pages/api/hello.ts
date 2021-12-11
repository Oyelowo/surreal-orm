// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import { getLowo } from "@oyelowo/lib-core";
import type { NextApiRequest, NextApiResponse } from "next";

type Data = {
  name: string;
  greetings: string;
};

export default function handler(req: NextApiRequest, res: NextApiResponse<Data>) {
  // dependencies across child packages
  const out = getLowo();

  res.status(200).json({ name: "John Doe", greetings: out });
}
