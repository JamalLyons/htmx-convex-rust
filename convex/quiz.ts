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
    await ctx.db.delete(args.quizID);
    return true;
  },
});

export const list = query({
  async handler(ctx) {
    return await ctx.db.query("quiz").collect();
  },
});

export const markComplete = mutation({
  args: { quizID: v.id("quiz") },
  async handler(ctx, args) {
    await ctx.db.patch(args.quizID, { complete: true });
    return true;
  },
});

export const unMarkComplete = mutation({
  args: { quizID: v.id("quiz") },
  async handler(ctx, args) {
    await ctx.db.patch(args.quizID, { complete: false });
    return true;
  },
});

const QUIZZES = [
  {
    subject: "Rust Fundamentals",
    name: "Ownership and Borrowing",
    points: 15,
    complete: true,
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
    complete: false,
    desc: "Learn about HTMX attributes and basic Ajax patterns.",
    questions: [
      {
        text: "Which attribute is used to trigger an AJAX request in HTMX when clicking an element?",
        options: ["hx-click", "hx-post", "hx-trigger", "hx-get"],
        correct_answer: 2,
      },
      {
        text: "What is the default event that triggers an hx-get request on an input element?",
        options: ["click", "change", "keyup", "submit"],
        correct_answer: 1,
      },
      {
        text: "Which attribute specifies where the response should be inserted in the DOM?",
        options: ["hx-target", "hx-swap", "hx-insert", "hx-place"],
        correct_answer: 0,
      },
      {
        text: "What does the 'hx-swap' attribute control?",
        options: [
          "The HTTP method to use",
          "Where to send the request",
          "How the response content is swapped into the DOM",
          "Which element triggers the request",
        ],
        correct_answer: 2,
      },
      {
        text: "Which indicator class is automatically added during an HTMX request?",
        options: [
          "htmx-requesting",
          "htmx-loading",
          "htmx-active",
          "htmx-pending",
        ],
        correct_answer: 1,
      },
    ],
  },
  {
    subject: "Database",
    name: "Convex Fundamentals",
    points: 12,
    complete: false,
    desc: "Explore the basics of Convex database and its features.",
    questions: [
      {
        text: "What type of database is Convex?",
        options: [
          "SQL Database",
          "Document Database",
          "Graph Database",
          "Key-Value Store",
        ],
        correct_answer: 1,
      },
      {
        text: "Which of these is a valid Convex query function declaration?",
        options: [
          "export const myQuery = query(() => {})",
          "export const myQuery = query.handler(() => {})",
          "export const myQuery = query({ handler: async (ctx) => {} })",
          "export const myQuery = async function query() {}",
        ],
        correct_answer: 2,
      },
      {
        text: "How does Convex handle real-time updates?",
        options: [
          "Through polling",
          "Using WebSocket subscriptions",
          "With server-sent events",
          "Through HTTP long polling",
        ],
        correct_answer: 1,
      },
      {
        text: "What is the purpose of the 'mutation' functions in Convex?",
        options: [
          "To query data only",
          "To modify the database",
          "To validate schemas",
          "To handle authentication",
        ],
        correct_answer: 1,
      },
      {
        text: "How does Convex ensure data consistency?",
        options: [
          "It doesn't guarantee consistency",
          "Through optimistic concurrency control",
          "Using global locks",
          "By limiting write operations",
        ],
        correct_answer: 1,
      },
    ],
  },
  {
    subject: "Rust Web",
    name: "Actix-web Essentials",
    points: 20,
    complete: false,
    desc: "Master the fundamentals of the Actix-web framework.",
    questions: [
      {
        text: "What trait must be implemented for a type to be used as a handler in Actix-web?",
        options: ["Handler", "Responder", "AsyncHandler", "WebHandler"],
        correct_answer: 1,
      },
      {
        text: "How do you extract path parameters in Actix-web?",
        options: [
          "Using the Query extractor",
          "Using the Path extractor",
          "Using request.params()",
          "Using URL.searchParams",
        ],
        correct_answer: 1,
      },
      {
        text: "Which macro is used to define a route that handles POST requests?",
        options: ["#[post]", "#[route]", "#[handler]", "#[web::post]"],
        correct_answer: 0,
      },
      {
        text: "What is the default worker thread count in Actix-web?",
        options: [
          "Number of CPU cores",
          "Single thread",
          "10 threads",
          "4 threads",
        ],
        correct_answer: 0,
      },
      {
        text: "Which middleware provides compression in Actix-web?",
        options: ["Compress", "Gzip", "Deflate", "DefaultHeaders"],
        correct_answer: 0,
      },
    ],
  },
  {
    subject: "Frontend",
    name: "Tailwind CSS Mastery",
    points: 8,
    complete: false,
    desc: "Learn essential Tailwind CSS utilities and patterns.",
    questions: [
      {
        text: "Which utility class adds padding to all sides of an element?",
        options: ["p", "pad", "padding", "p-all"],
        correct_answer: 0,
      },
      {
        text: "What is the correct way to specify a flex container in Tailwind?",
        options: ["flex", "display-flex", "d-flex", "flexbox"],
        correct_answer: 0,
      },
      {
        text: "Which class makes an element's position fixed?",
        options: ["position-fixed", "pos-fixed", "fixed", "static"],
        correct_answer: 2,
      },
      {
        text: "How do you make text bold using Tailwind?",
        options: ["text-bold", "font-bold", "fw-bold", "bold"],
        correct_answer: 1,
      },
      {
        text: "Which class adds a shadow to an element?",
        options: ["shadow", "box-shadow", "drop-shadow", "elevation"],
        correct_answer: 0,
      },
    ],
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
      return false;
    }

    const docs = await ctx.db.query("quiz").collect();
    console.log(`Found ${docs.length} existing quizzes`);

    for (const doc of docs) {
      await ctx.db.delete(doc._id);
    }
    console.log("Deleted all existing quizzes");

    for (let quiz of QUIZZES) {
      const id = await ctx.db.insert("quiz", quiz);
      console.log(`Inserted quiz: ${quiz.name} with ID: ${id}`);
    }

    // Verify the insertion
    const newDocs = await ctx.db.query("quiz").collect();
    console.log(`Total quizzes after seeding: ${newDocs.length}`);

    return true;
  },
});
