"use client";
import { useState } from "react";
import PostLister from "./components/post-lister";
import PostForm from "./components/post-form";
import { PostWithUsername } from "./components/post";

export default function Home() {
  const [posts, setPosts] = useState<PostWithUsername[]>([]);

  return (
    <div
      className={
        "flex flex-col items-center justify-center pt-20 gap-5 bg-background text-accent min-h-screen w-screen"
      }
    >
      <h1 className={"text-2xl font-bold"}>Portable Media</h1>
      <PostForm setPosts={setPosts} posts={posts} />
      <h2 className={"mt-20 text-2xl font-bold text-accent"}>Posts:</h2>
      <PostLister posts={posts} setPosts={setPosts} />
    </div>
  );
}
