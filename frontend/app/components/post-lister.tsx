import { useEffect, useState } from "react";
import Post from "./post";
import { PostType } from "./post";

export default function PostLister() {
  const [posts, setPosts] = useState<PostType[]>([]);

  const fetchPosts = async () => {
    const response = await fetch("http://localhost:8000/api/list_posts");
    if (!response.ok) {
      console.error("NOOOO! ERROR GETTING POSTS!");
      return;
    }

    const postResponse: PostType[] = (await response.json()).body.Posts;
    setPosts(postResponse.reverse());
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  return (
    <>
      {posts.map((post) => {
        return (
          <Post
            username={post.username}
            time={post.time}
            text={post.text}
            post_id={post.post_id}
            key={post.post_id}
            setPosts={setPosts}
          />
        );
      })}
    </>
  );
}
