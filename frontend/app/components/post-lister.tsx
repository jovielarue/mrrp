import { useEffect, useState } from "react";
import Post, { PostWithUsername } from "./post";
import { PostType } from "./post";

export default function PostLister() {
  const [posts, setPosts] = useState<PostWithUsername[]>([]);

  const fetchPosts = async () => {
    const response = await fetch("http://localhost:8000/api/list_posts");
    if (!response.ok) {
      console.error("NOOOO! ERROR GETTING POSTS!");
      return;
    }

    const posts: PostWithUsername[] = (await response.json()).body.PostReturns;
    console.log(posts[0].post.time);

    setPosts(posts.reverse());
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
            post={post.post}
            key={post.post.post_id}
            setPosts={setPosts}
          />
        );
      })}
    </>
  );
}
