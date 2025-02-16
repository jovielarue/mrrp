import { Dispatch, SetStateAction } from "react";
export type PostType = {
  username: string;
  time: string;
  text: string;
  post_id: string;
  setPosts: Dispatch<SetStateAction<PostType[]>>;
};

export default function Post(props: PostType) {
  const date = new Date(props.time).toLocaleString();
  const handleDelete = async () => {
    const response = await fetch(
      "http://localhost:8000/api/delete/" + props.post_id,
    );
    if (response.ok) {
      console.log("Deleted post. Refresh page to see result of deletion.");
    } else {
      console.error("Error deleting post.");
    }
  };

  return (
    <div className={"w-[25rem]"}>
      <div className={"flex flex-col bg-primary p-5 text-background"}>
        <div className={"flex justify-between"}>
          <p className={"font-bold"}>{props.username}</p>
          <button className={"bg-accent2 px-2 py-1"} onClick={handleDelete}>
            delete
          </button>
        </div>
        <p className={"text-sm"}>{date}</p>
        <p className={"text-lg"}>{props.text}</p>
      </div>
    </div>
  );
}
