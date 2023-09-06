# Flair

An augmentation for [Lemmy](https://join-lemmy.org) that adds user flair support to the backend. 


## API Reference

#### Get a user's flairs

```http
  GET /user
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `id` | `number` | **Required**. Lemmy user ID |

#### Add a user to database

```http
  POST /user
```

**Body**
```jsonc
{
    special: bool,
    ref_id: string,
    pos: number
    flair: number // Reference to flair ID
    path?: string
}
```



