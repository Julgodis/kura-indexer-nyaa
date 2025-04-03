import { queryOptions } from "@tanstack/react-query"
import { ListRequest, ListResponseSchema, MirrorResponseSchema } from "./types"
import { ApiUrl } from "./url"

export const mirrorQueryOptions = queryOptions({
    queryKey: ['mirror'],
    queryFn: async () => {
        const response = await fetch(`${ApiUrl}/api/mirror`)
        const data = await response.json()
        return MirrorResponseSchema.parse(data)
    }
})

export const listQueryOptions = (mirror: string, search: ListRequest) => {
    return queryOptions({
        queryKey: ['list', mirror, search.p, search.c, search.s, search.o, search.f, search.q],
        staleTime: 1000 * 60 * 5, // 5 minutes
        queryFn: async () => {
            const searchParams = new URLSearchParams()
            if (search.p) searchParams.append('p', search.p.toString())
            if (search.c) searchParams.append('c', search.c.toString())
            if (search.s) searchParams.append('s', search.s.toString())
            if (search.o) searchParams.append('o', search.o.toString())
            if (search.f) searchParams.append('f', search.f.toString())
            if (search.q) searchParams.append('q', search.q.toString())

            const response = await fetch(`${ApiUrl}/api/mirror/${mirror}/list?${searchParams.toString()}`)
            const data = await response.json()
            return ListResponseSchema.parse(data)
        }
    })
}
