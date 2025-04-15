"use client";
import {
  Dispatch,
  FormEvent,
  SetStateAction,
  useContext,
  useState,
} from "react";
import { PostWithUsername } from "./post";
import { UserContext } from "../contexts/usercontext";

interface IPostFormType {
  posts: PostWithUsername[];
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
}

export default function PostForm(props: IPostFormType) {
  const [postText, setPostText] = useState<string>("");
  const [postAlert, setPostAlert] = useState<boolean>(false);
  const { handleGetUsername, handleGetAuthToken } = useContext(UserContext);

  const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setPostAlert(false);
    try {
      const postData = new FormData();
      postData.set("post", postText);
      postData.set("username", handleGetUsername());
      postData.set("jwt", handleGetAuthToken());

      if (postData.get("jwt") === "" || postData.get("username") === "") {
        return;
      }

      const response = await fetch("http://localhost:8000/api/new_post", {
        body: postData,
        method: "POST",
      });

      if (!response.ok) {
        setPostAlert(true);
        return;
      }

      const postResponse = await response.json();
      props.setPosts([postResponse, ...props.posts]);

      setPostText("");
    } catch (e) {
      console.error("There was an error creating this post: " + e);
    }
  };

  return (
    <form
      className={"flex flex-col gap-3 w-full p-4"}
      onSubmit={(e) => handleSubmit(e)}
    >
      <div className={"flex flex-col gap-1 w-full"}>
        <label htmlFor="post" className={"text-xl"}>
          mrrp text
        </label>
        <textarea
          className={
            "bg-secondary text-background placeholder-background p-2 rounded-sm text-lg w-full"
          }
          name="post"
          placeholder="mrrp goes here..."
          value={postText}
          onChange={(e) => {
            setPostText(e.target.value);
          }}
        />
      </div>
      <button
        className={
          "hover:bg-accent bg-secondary hover:text-background text-accent py-1 px-3 rounded-sm"
        }
        type="submit"
      >
        Post!
      </button>
      {postAlert && (
        <p className={"text-accent2"}>
          Your post was detected as being AI-generated. Please only post
          human-generated content.
        </p>
      )}
    </form>
  );
}
