"use client";
import { Dispatch, SetStateAction, useEffect, useState } from "react";
import Post, { PostWithUsername } from "./post";
interface IPostLister {
  posts: PostWithUsername[];
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
}

export default function PostLister(props: IPostLister) {
  const [editing, setEditing] = useState<string>("");

  const fetchPosts = async () => {
    try {
      const response = await fetch("http://localhost:8000/api/list_posts");
      if (!response.ok) {
        console.error("NOOOO! ERROR GETTING POSTS!");
        return;
      }

      const posts: PostWithUsername[] = (await response.json()).body
        .PostReturns;
      console.log(posts);

      props.setPosts(posts);
    } catch (e) {
      console.error("There was an error fetching posts: " + e);
    }
  };

  const handleEdit = (id: string, toEdit: boolean) => {
    console.log(id);
    setEditing(toEdit ? id : "");
  };

  useEffect(() => {
    fetchPosts();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div
      className={"flex flex-col items-center justify-center gap-4 p-4 w-full"}
    >
      {props.posts.map((post) => {
        return (
          <Post
            postWithUsername={post}
            key={post.post.post_id}
            setPosts={props.setPosts}
            posts={props.posts}
            handleEdit={handleEdit}
            editing={post.post.post_id === editing ? true : false}
          />
        );
      })}
    </div>
  );
}
