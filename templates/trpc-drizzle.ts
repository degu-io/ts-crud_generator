import { z } from "zod";

import { createTRPCRouter, publicProcedure } from "@/server/api/trpc";
import { {{ENTITY_NAME}} } from "@/server/db/schema";
import { eq } from "drizzle-orm";

export const {{ENTITY_NAME}}Router = createTRPCRouter({
  create: publicProcedure
    .input(z.object({ projectId: z.number(), name: z.string().min(1) }))
    .mutation(async ({ ctx, input }) => {
      await ctx.db.insert({{ENTITY_NAME}}).values({
        projectId: input.projectId,
        name: input.name,
      });
    }),

  getAll: publicProcedure.query(({ ctx }) => {
    return ctx.db.query.{{ENTITY_NAME}}.findMany();
  }),

  getById: publicProcedure
    .input(z.object({ id: z.number() }))
    .query(({ ctx, input }) => {
      return ctx.db.query.{{ENTITY_NAME}}.findFirst({
        where: eq({{ENTITY_NAME}}.id, input.id),
        with: {
          assets: true,
          project: { columns: { name: true } },
        },
      });
    }),
});
