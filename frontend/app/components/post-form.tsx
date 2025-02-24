import { Dispatch, SetStateAction, useState } from "react";
import { PostWithUsername } from "./post";

interface IPostFormType {
  posts: PostWithUsername[];
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
}

export default function PostForm(props: IPostFormType) {
  const [username, setUsername] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [postText, setPostText] = useState<string>("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const formData = new FormData(e.target);
      const response = await fetch("http://localhost:8000/api/new_post", {
        body: formData,
        method: "POST",
      });

      const postResponse = (await response.json()).body.Post;
      console.log(postResponse);
      props.setPosts([postResponse, ...props.posts]);
      setUsername("");
      setPassword("");
      setPostText("");
    } catch (e) {
      console.error("There was an error creating this post: " + e);
    }
  };

  return (
    <form className={"flex flex-col gap-3"} onSubmit={(e) => handleSubmit(e)}>
      <div className={"flex flex-col"}>
        <label htmlFor="username">Username</label>
        <input
          className={
            "bg-secondary text-background placeholder-background p-2 rounded-sm"
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
            "bg-secondary text-background placeholder-background p-2 rounded-sm"
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
            "bg-secondary text-background placeholder-background p-2 rounded-sm"
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
        className={
          "hover:bg-accent2 bg-secondary hover:text-background text-accent py-1 px-3 rounded-sm"
        }
        type="submit"
      >
        Post!
      </button>
    </form>
  );
}
