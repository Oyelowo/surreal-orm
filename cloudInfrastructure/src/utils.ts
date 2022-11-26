import z from "zod";

export const appEnvironmentsSchema = z.union([
	z.literal("test"),
	z.literal("local"),
	z.literal("development"),
	z.literal("staging"),
	z.literal("production"),
]);

export type Environment = z.infer<typeof appEnvironmentsSchema>;
