import { open } from "@tauri-apps/plugin-shell";
import "./UpdateBanner.css";

interface UpdateBannerProps {
  currentVersion: string;
  latestVersion: string;
}

export default function UpdateBanner({
  currentVersion,
  latestVersion,
}: UpdateBannerProps) {
  const handleUpdate = async () => {
    await open("https://github.com/inyourface34456/Blur-AutoClicker/releases/latest");
  };

  return (
    <div className="update-banner">
      <span className="update-banner-text-old-version">v{currentVersion}</span>
      <span className="update-banner-text">→</span>
      <span className="update-banner-text-new-version">{latestVersion}</span>
      <button className="update-banner-btn" onClick={handleUpdate}>
        View Update
      </button>
    </div>
  );
}
