import markdown
from litestar import get

@get("/report/{target}")
async def get_markdown_report(target: str):
    with open(f"/data/{target}.md", "r") as f:
        md_content = f.read()
    html_content = markdown.markdown(md_content)
    return {"html": html_content}
