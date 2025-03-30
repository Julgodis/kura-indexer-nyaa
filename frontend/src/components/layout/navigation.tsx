import { Link } from "@tanstack/react-router";
import { NavigationMenu, NavigationMenuItem, NavigationMenuLink, NavigationMenuList, navigationMenuTriggerStyle } from "../ui/navigation-menu";

export function Navigation() {
    const show_stats = false; // Replace with actual condition to show stats

    return (
        <NavigationMenu>
            <NavigationMenuList>
                <NavigationMenuItem>
                    <NavigationMenuLink className={navigationMenuTriggerStyle()} asChild>
                        <Link to="/" search={{}}>
                            Torrents
                        </Link>
                    </NavigationMenuLink>
                </NavigationMenuItem>
                {show_stats && (
                    <NavigationMenuItem>
                        <NavigationMenuLink className={navigationMenuTriggerStyle()} asChild>
                            <Link to="/stats">
                                Statistics
                            </Link>
                        </NavigationMenuLink>
                    </NavigationMenuItem>
                )}
            </NavigationMenuList>
        </NavigationMenu>
    );
}