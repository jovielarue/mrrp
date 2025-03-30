"use client";
import { useContext, useEffect, useState } from "react";
import PostLister from "./components/post-lister";
import PostForm from "./components/post-form";
import { PostWithUsername } from "./components/post";
import { useRouter } from "next/navigation";
import { UserContext } from "./contexts/usercontext";

export default function Home() {
  const [posts, setPosts] = useState<PostWithUsername[]>([]);
  const { handleGetUsername, handleSetUsername } = useContext(UserContext);
  const router = useRouter();

  const logout = () => {
    handleSetUsername("");
    router.push("/auth");
  };

  useEffect(() => {
    if (handleGetUsername() === "") {
      router.push("/auth");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className={"w-full flex flex-col items-center justify-center gap-10"}>
      <h1 className={"text-4xl font-bold"}>mrrp</h1>
      <div className={"flex flex-col items-center justify-center gap-3"}>
        <h2 className={"text-2xl font-bold"}>
          welcome, {handleGetUsername()}!
        </h2>
        <button
          className={
            "text-sm bg-secondary text-accent hover:bg-accent2 px-2 py-1 rounded-md"
          }
          onClick={logout}
        >
          log out
        </button>
      </div>
      <PostForm setPosts={setPosts} posts={posts} />
      <div className={"flex flex-col items-center justify-center w-full gap-3"}>
        <h2 className={"text-2xl font-bold text-accent"}>Posts:</h2>
        <PostLister posts={posts} setPosts={setPosts} />
      </div>
    </div>
  );
}
