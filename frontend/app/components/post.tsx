import { Dispatch, SetStateAction } from "react";
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
  setPosts: Dispatch<SetStateAction<PostWithUsername[]>>;
};

export default function Post(props: PostWithUsername) {
  const date = new Date(props.post.time).toLocaleString();
  const handleDelete = async () => {
    const response = await fetch(
      "http://localhost:8000/api/delete/" + props.post.post_id,
    );
    if (response.ok) {
      console.log("Deleted post. Refresh page to see result of deletion.");
    } else {
      console.error("Error deleting post.");
    }
  };

  return (
    <div
      className={
        "w-[25rem] flex justify-between bg-primary p-5 text-background"
      }
    >
      <div className={"flex flex-col"}>
        <div className={"flex justify-between"}>
          <p className={"font-bold"}>{props.username}</p>
        </div>
        <p className={"text-sm"}>{date}</p>
        <p className={"text-lg"}>{props.post.text}</p>
      </div>
      <div className={"flex flex-col items-end justify-center gap-2"}>
        <button className={"bg-accent2 px-2 py-1"} onClick={handleDelete}>
          delete
        </button>
        <p>#{props.post.post_id}</p>
      </div>
    </div>
  );
}
