import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { AlertTriangle } from "lucide-react";


interface ErrorProps {
    error?: Error | { message: string } | string;
    title?: string;
    onRetry?: () => void;
}

export function ErrorCard({ error, title = "An error occurred", onRetry }: ErrorProps) {
    const errorMessage = typeof error === 'string'
        ? error
        : error?.message || "Something went wrong. Please try again later.";

    return (
        <div className="flex items-center justify-center min-h-[50vh]">
            <Card className="w-full max-w-md shadow-lg">
                <CardHeader className="flex flex-row items-center gap-2 pb-2">
                    <AlertTriangle className="h-5 w-5 text-destructive" />
                    <CardTitle>{title}</CardTitle>
                </CardHeader>
                <CardContent>
                    <CardDescription className="text-destructive text-sm mt-2 whitespace-pre-wrap break-words">
                        <code>{errorMessage}</code>
                    </CardDescription>
                </CardContent>
                {onRetry && (
                    <CardFooter>
                        <Button variant="outline" onClick={onRetry} className="w-full">
                            Try Again
                        </Button>
                    </CardFooter>
                )}
            </Card>
        </div>
    );
}