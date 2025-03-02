"use client";
import { Dispatch, SetStateAction, useContext, useState } from "react";
import { UserContext } from "../contexts/usercontext";
export type PostType = {
  username: string;
  time: string;
  text: string;
  post_id: string;
  setPosts: Dispatch<SetStateAction<PostType[]>>;
};

export type PostWithUsername = {
  post: PostType;
  username: string;
};

interface IPost {
  postWithUsername: PostWithUsername;
  posts: PostWithUsername[];
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
  handleEdit: (id: string, toEdit: boolean) => void;
  editing: boolean;
}

export default function Post(props: IPost) {
  const date = new Date(props.postWithUsername.post.time).toLocaleString();
  const { handleGetUsername, handleGetAuthToken } = useContext(UserContext);
  const username = handleGetUsername();
  const [newText, setNewText] = useState<string>("");

  const handleDelete = async () => {
    try {
      const response = await fetch(
        "http://localhost:8000/api/delete/" +
          props.postWithUsername.post.post_id,
      );
      if (response.ok) {
        console.log("Deleted post.");
        const postsWithoutDeleted = props.posts.filter(
          (postWithUsername) =>
            postWithUsername.post.post_id !==
            props.postWithUsername.post.post_id,
        );
        props.setPosts(postsWithoutDeleted);
      } else {
        console.error("Error deleting post.");
      }
    } catch (e) {
      console.error("There was an error deleting the post: " + e);
    }
  };

  const handleSave = async () => {
    const postData = new FormData();
    postData.set("username", handleGetUsername());
    postData.set("jwt", handleGetAuthToken());
    postData.set("post", newText);

    try {
      const response = await fetch(
        "http://localhost:8000/api/edit/" + props.postWithUsername.post.post_id,
        { method: "PUT", body: postData },
      );
      if (response.ok) {
        console.log("Edited post.");
        const postsWithEdited = props.posts.map((postWithUsername) => {
          if (
            postWithUsername.post.post_id ===
            props.postWithUsername.post.post_id
          ) {
            return {
              ...postWithUsername,
              post: { ...postWithUsername.post, text: newText },
            };
          } else {
            return postWithUsername;
          }
        });
        props.setPosts(postsWithEdited);
        props.handleEdit(props.postWithUsername.post.post_id, !props.editing);
      } else {
        console.error("Error editing post.");
      }
    } catch (e) {
      console.error("There was an error deleting the post: " + e);
    }
  };

  return (
    <>
      {props.editing ? (
        <div
          className={
            "w-[25rem] flex justify-between bg-secondary p-5 text-accent rounded-sm"
          }
        >
          <div className={"flex flex-col gap-2"}>
            <div className={"flex justify-between"}>
              <p className={"font-bold"}>{props.postWithUsername.username}</p>
            </div>
            <p className={"text-sm"}>{date}</p>
            <input
              value={newText}
              onChange={(e) => setNewText(e.target.value)}
              className={
                "text-lg bg-background h-full placeholder:text-primary border-background border-2 rounded-sm flex items-center justify-center pl-2"
              }
              placeholder="new mrrp goes here..."
            />
          </div>
          <div className={"flex flex-col items-end justify-start gap-2"}>
            {username === props.postWithUsername.username && (
              <>
                <button
                  className={
                    "bg-background text-primary px-2 py-1 rounded-sm w-full"
                  }
                  onClick={handleSave}
                >
                  save
                </button>
                <button
                  className={"bg-accent2 px-2 py-1 rounded-sm"}
                  onClick={handleDelete}
                >
                  delete
                </button>
              </>
            )}
            <p className={"text-sm"}>
              mrrp #{props.postWithUsername.post.post_id}
            </p>
          </div>
        </div>
      ) : (
        <div
          className={
            "w-[25rem] flex justify-between bg-primary p-5 text-background rounded-sm"
          }
        >
          <div className={"flex flex-col"}>
            <div className={"flex justify-between"}>
              <p className={"font-bold"}>{props.postWithUsername.username}</p>
            </div>
            <p className={"text-sm"}>{date}</p>
            <p className={"text-lg"}>{props.postWithUsername.post.text}</p>
          </div>
          <div className={"flex flex-col items-end justify-start gap-2"}>
            {username === props.postWithUsername.username && (
              <>
                <button
                  className={
                    "bg-background text-primary px-2 py-1 rounded-sm w-full"
                  }
                  onClick={() =>
                    props.handleEdit(
                      props.postWithUsername.post.post_id,
                      !props.editing,
                    )
                  }
                >
                  edit
                </button>
              </>
            )}
            <p className={"text-sm"}>
              mrrp #{props.postWithUsername.post.post_id}
            </p>
          </div>
        </div>
      )}
    </>
  );
}
