import { Doc } from "./_generated/dataModel";
import { internalMutation, mutation, query } from "./_generated/server";
import { v } from "convex/values";

export const get = query({
  args: {
    quizID: v.id("quiz"),
  },
  async handler(ctx, args) {
    return await ctx.db.get(args.quizID);
  },
});

export const remove = mutation({
  args: {
    quizID: v.id("quiz"),
  },
  async handler(ctx, args) {
    return await ctx.db.delete(args.quizID);
  },
});

export const list = query({
  async handler(ctx) {
    return await ctx.db.query("quiz").collect();
  },
});

const QUIZZES = [
  {
    subject: "Rust Fundamentals",
    name: "Ownership and Borrowing",
    points: 15,
    complete: false,
    desc: "Test your knowledge of Rust's ownership system and borrowing rules.",
    questions: [
      {
        text: "What is the primary benefit of Rust's ownership system?",
        options: [
          "Memory safety without garbage collection",
          "Automatic memory management with GC",
          "Dynamic typing capabilities",
          "Interpreted language features",
        ],
        correct_answer: 0,
      },
      {
        text: "In Rust, how many owners can a piece of data have at a time?",
        options: ["As many as needed", "Two", "One", "Zero"],
        correct_answer: 2,
      },
      {
        text: "What happens when a variable goes out of scope in Rust?",
        options: [
          "Nothing",
          "The memory is automatically freed",
          "It becomes null",
          "It's garbage collected",
        ],
        correct_answer: 1,
      },
      {
        text: "Which of these is a mutable borrow?",
        options: ["&self", "&'static str", "&mut String", "String"],
        correct_answer: 2,
      },
      {
        text: "What is the purpose of the 'move' keyword in Rust?",
        options: [
          "To copy data",
          "To force ownership transfer",
          "To create a reference",
          "To delete data",
        ],
        correct_answer: 1,
      },
    ],
  },
  {
    subject: "Web Development",
    name: "HTMX Basics",
    points: 10,
    complete: true,
    desc: "Learn about HTMX attributes and basic Ajax patterns.",
    questions: [],
  },
  {
    subject: "Database",
    name: "Convex Fundamentals",
    points: 12,
    complete: true,
    desc: "Explore the basics of Convex database and its features.",
    questions: [],
  },
  {
    subject: "Rust Web",
    name: "Actix-web Essentials",
    points: 20,
    complete: true,
    desc: "Master the fundamentals of the Actix-web framework.",
    questions: [],
  },
  {
    subject: "Frontend",
    name: "Tailwind CSS Mastery",
    points: 8,
    complete: true,
    desc: "Learn essential Tailwind CSS utilities and patterns.",
    questions: [],
  },
] as Array<Doc<"quiz">>;

export const seedQuizTable = internalMutation({
  args: {
    agree: v.optional(v.boolean()),
  },
  async handler(ctx, args) {
    if (!args.agree) {
      console.log(`WARNING THIS FUNCTION WILL DELETE ALL CURRENT DATA`);
      console.log(`Please args.agree true to continue...`);
      return;
    }

    const docs = await ctx.db.query("quiz").collect();

    for (const doc of docs) {
      await ctx.db.delete(doc._id);
    }

    for (let quiz of QUIZZES) {
      await ctx.db.insert("quiz", quiz);
    }
  },
});
