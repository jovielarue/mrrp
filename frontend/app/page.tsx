"use client";
import { useState } from "react";
import PostLister from "./components/post-lister";

export default function Home() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [postText, setPostText] = useState<string>("");
  console.log("here");

  const handleSubmit = async (e) => {
    e.preventDefault();
    console.log("here");
    const formData = new FormData(e.target);
    const response = await fetch("http://localhost:8000/api/new_post", {
      body: formData,
      method: "POST",
    });
    console.log(response);
  };

  return (
    <div
      className={
        "flex flex-col items-center justify-center pt-20 gap-5 bg-background text-accent min-h-screen w-screen"
      }
    >
      <h1 className={"text-2xl font-bold"}>Portable Media</h1>
      <form className={"flex flex-col gap-3"} onSubmit={(e) => handleSubmit(e)}>
        <div className={"flex flex-col"}>
          <label htmlFor="username">Username</label>
          <input
            className={
              "bg-secondary text-background placeholder-background p-2"
            }
            name="username"
            type="text"
            placeholder="username goes here..."
            value={username}
            onChange={(e) => {
              setUsername(e.target.value);
            }}
          />
        </div>
        <div className={"flex flex-col"}>
          <label htmlFor="password">Password</label>
          <input
            className={
              "bg-secondary text-background placeholder-background p-2"
            }
            name="password"
            type="password"
            placeholder="password goes here..."
            value={password}
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
        </div>
        <div className={"flex flex-col"}>
          <label htmlFor="post">Post text</label>
          <textarea
            className={
              "bg-secondary text-background placeholder-background p-2"
            }
            name="post"
            placeholder="post text goes here..."
            value={postText}
            onChange={(e) => {
              setPostText(e.target.value);
            }}
          />
        </div>
        <button
          className={"bg-accent2 text-background py-1 px-3"}
          type="submit"
        >
          Post!
        </button>
      </form>
      <h2 className={"mt-20 text-2xl font-bold text-accent"}>Posts:</h2>
      <PostLister />
    </div>
  );
}
