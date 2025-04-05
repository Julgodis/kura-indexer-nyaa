import { queryOptions } from "@tanstack/react-query"
import { ListRequest, ListResponseSchema, MirrorHealthResponseSchema, MirrorResponseSchema } from "./types"
import { ApiUrl } from "./url"

export const mirrorQueryOptions = queryOptions({
    queryKey: ['mirror'],
    queryFn: async () => {
        const response = await fetch(`${ApiUrl}/api/mirror`)
        return MirrorResponseSchema.parse(await response.json())
    }
})

export const listQueryOptions = (mirror: string, search: ListRequest) => {
    return queryOptions({
        queryKey: ['list', mirror, search.p, search.c, search.s, search.o, search.f, search.q],
        staleTime: 1000 * 60 * 5, // 5 minutes
        queryFn: async () => {
            const searchParams = new URLSearchParams()
            if (search.p) searchParams.append('p', search.p.toString())
            searchParams.append('c', search.c.toString())
            searchParams.append('s', search.s.toString())
            searchParams.append('o', search.o.toString())
            searchParams.append('f', search.f.toString())
            if (search.q) searchParams.append('q', search.q.toString())

            const response = await fetch(`${ApiUrl}/api/mirror/${mirror}/list?${searchParams.toString()}`)
            return ListResponseSchema.parse(await response.json())
        }
    })
}

export const healthQueryOptions = queryOptions({
    queryKey: ['health'],
    queryFn: async () => {
        const response = await fetch(`${ApiUrl}/api/health`)
        return MirrorHealthResponseSchema.parse(await response.json())
    }
})
