import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  quiz: defineTable({
    subject: v.string(),
    name: v.string(),
    desc: v.string(),
    points: v.number(),
    complete: v.boolean(),
    questions: v.array(
      v.object({
        text: v.string(),
        options: v.array(v.string()),
        correct_answer: v.number(),
      })
    ),
  }),
});
