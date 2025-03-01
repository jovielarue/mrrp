import { Dispatch, SetStateAction, useContext, useState } from "react";
import { PostWithUsername } from "./post";
import { UserContext } from "../contexts/usercontext";

interface IPostFormType {
  posts: PostWithUsername[];
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
}

export default function PostForm(props: IPostFormType) {
  const [postText, setPostText] = useState<string>("");
  const { handleGetUsername, handleGetAuthToken } = useContext(UserContext);

  const handleSubmit = async (e) => {
    e.preventDefault();
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

      const postResponse = await response.json();
      props.setPosts([postResponse, ...props.posts]);

      setPostText("");
    } catch (e) {
      console.error("There was an error creating this post: " + e);
    }
  };

  return (
    <form className={"flex flex-col gap-3"} onSubmit={(e) => handleSubmit(e)}>
      <div className={"flex flex-col gap-1"}>
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
