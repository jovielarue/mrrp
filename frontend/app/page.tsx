"use client";
import { useContext, useEffect, useState } from "react";
import PostLister from "./components/post-lister";
import PostForm from "./components/post-form";
import { PostWithUsername } from "./components/post";
import { useRouter } from "next/navigation";
import { UserContext } from "./contexts/usercontext";

export default function Home() {
  const [posts, setPosts] = useState<PostWithUsername[]>([]);
  const { handleGetUsername } = useContext(UserContext);
  const router = useRouter();

  useEffect(() => {
    console.log(handleGetUsername());
    if (handleGetUsername() === "") {
      router.push("/auth");
    }
  }, []);

  return (
    <>
      <h1 className={"text-2xl font-bold"}>mrrp</h1>
      <PostForm setPosts={setPosts} posts={posts} />
      <h2 className={"mt-20 text-2xl font-bold text-accent"}>Posts:</h2>
      <PostLister posts={posts} setPosts={setPosts} />
    </>
  );
}
