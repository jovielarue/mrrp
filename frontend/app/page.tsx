"use client";
import { useState } from "react";

export default function Home() {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [postText, setPostText] = useState<string>("");
  console.log("here");

  const handleSubmit = async (e) => {
    e.preventDefault();
    console.log("here");
    const formData = new FormData(e.target);
    const response = await fetch("http://127.0.0.1:8000/api/new_post", {
      body: formData,
      method: "POST",
    });
    console.log(response);
  };

  return (
    <div
      className={
        "flex flex-col items-center justify-center gap-5 bg-background text-accent h-screen w-screen"
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
          className={"bg-primary text-background py-1 px-3"}
          type="submit"
        >
          Post!
        </button>
      </form>
    </div>
  );
}
