import { mutation, query } from "./_generated/server";
import { v } from "convex/values";

export const getPlayerScore = query({
  args: {},
  handler: async (ctx) => {
    const player = await ctx.db.query("player").first();
    return player?.score ?? 0;
  },
});

export const updatePlayerScore = mutation({
  args: {
    pointsChange: v.number(),
  },
  handler: async (ctx, args) => {
    const player = await ctx.db.query("player").first();

    if (player) {
      await ctx.db.patch(player._id, {
        score: player.score + args.pointsChange,
      });
    } else {
      await ctx.db.insert("player", {
        score: args.pointsChange,
      });
    }
  },
});

export const seedPlayer = mutation({
  args: {},
  handler: async (ctx) => {
    const player = await ctx.db.query("player").first();
    if (player) {
      await ctx.db.delete(player._id);
      return;
    }

    await ctx.db.insert("player", {
      score: 0,
    });
  },
});
