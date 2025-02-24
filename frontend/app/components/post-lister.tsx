import { Dispatch, SetStateAction, useEffect } from "react";
import Post, { PostWithUsername } from "./post";
interface IPostLister {
  posts: PostWithUsername[];
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
}

export default function PostLister(props: IPostLister) {
  const fetchPosts = async () => {
    const response = await fetch("http://localhost:8000/api/list_posts");
    if (!response.ok) {
      console.error("NOOOO! ERROR GETTING POSTS!");
      return;
    }

    const posts: PostWithUsername[] = (await response.json()).body.PostReturns;
    console.log(posts[0].post.time);

    props.setPosts(posts);
  };

  useEffect(() => {
    fetchPosts();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className={"flex flex-col gap-2 p-4"}>
      {props.posts.map((post) => {
        return (
          <Post
            postWithUsername={post}
            key={post.post.post_id}
            setPosts={props.setPosts}
            posts={props.posts}
          />
        );
      })}
    </div>
  );
}
