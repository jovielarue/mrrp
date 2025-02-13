export type PostType = {
  username: string;
  time: string;
  text: string;
  id: string;
};

export default function Post(props: PostType) {
  const date = new Date(props.time).toLocaleString();

  return (
    <div className={"w-[25rem]"}>
      <div className={"flex flex-col bg-primary p-5 text-background"}>
        <p className={"font-bold"}>{props.username}</p>
        <p className={"text-sm"}>{date}</p>
        <p className={"text-lg"}>{props.text}</p>
      </div>
    </div>
  );
}
