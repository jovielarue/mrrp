"use client";
import { useState } from "react";
import PostLister from "./components/post-lister";
import PostForm from "./components/post-form";
import { PostWithUsername } from "./components/post";

export default function Home() {
  const [posts, setPosts] = useState<PostWithUsername[]>([]);

  return (
    <>
      <h1 className={"text-2xl font-bold"}>mrrp</h1>
      <PostForm setPosts={setPosts} posts={posts} />
      <h2 className={"mt-20 text-2xl font-bold text-accent"}>Posts:</h2>
      <PostLister posts={posts} setPosts={setPosts} />
    </>
  );
}
